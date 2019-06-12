import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import org.openqa.selenium.By as By
import org.openqa.selenium.WebElement as WebElement
import com.kms.katalon.core.annotation.Keyword as Keyword
import com.kms.katalon.core.util.KeywordUtil as KeywordUtil

WebUI.callTestCase(findTestCase('BEMUTATO_onalloan_futtathatok/TC-Login'), null, FailureHandling.STOP_ON_FAILURE)

WebUI.waitForElementVisible(findTestObject('Test_Login/span_Pcienstrzs'), 5)

WebUI.verifyTextPresent('Pácienstörzs', false, FailureHandling.OPTIONAL)

//klikk a paciensfelv gombra
WebUI.click(findTestObject('PacienstorzsSearch/btnUjPaciens'))

WebUI.waitForElementPresent(findTestObject('PaciensFelvetelForm/input_Nev'), 10)

WebUI.click(findTestObject('PaciensFelvetelForm/btnMentes'))



//classok lekérdezése
String nevCss = WebUI.getCSSValue(findTestObject('PaciensFelvetelForm/input_Nev'), 'background')

String azonCss = WebUI.getCSSValue(findTestObject('PaciensFelvetelForm/input_Azonosito'), 'background')

String szuldatCss = WebUI.getCSSValue(findTestObject('PaciensFelvetelForm/input_SzuletesiDatum'), 'background')

String irszCss = WebUI.getCSSValue(findTestObject('PaciensFelvetelForm/input_Iranyitoszam'), 'background')

String cimCss = WebUI.getCSSValue(findTestObject('PaciensFelvetelForm/input_Cim'), 'background')

//------------------
KeywordUtil.markWarning('A nevCss is : ' + nevCss)

KeywordUtil.markWarning('Az azonCss is : ' + azonCss)

KeywordUtil.markWarning('A szuldatCss is : ' + szuldatCss)

KeywordUtil.markWarning('A irszCss is : ' + irszCss)

KeywordUtil.markWarning('A cimCss is : ' + cimCss)
//----------------------

CharSequence seq = "rgb(230, 102, 102)";

boolean bool1 = nevCss.contains(seq)
boolean bool2 = azonCss.contains(seq)
boolean bool3 = szuldatCss.contains(seq)
boolean bool4 = irszCss.contains(seq)
boolean bool5 = cimCss.contains(seq)

if(bool1 && bool2 && bool3 && bool4 && bool5){
	KeywordUtil.markPassed('Az összes hatter rendben van!')
}else{
KeywordUtil.markFailed('A hatterszín nem stimmel!')
}





